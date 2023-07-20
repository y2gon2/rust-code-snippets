//! graph dijkstra method 구축을 통해 특정 시작 node 에서 부터 다른 전체 노드까지의
//! 최단 거리 확인 
//! 구현 배경은 백준 최단경로 문제(https://www.acmicpc.net/problem/1753) 을 기준으로 함.


use std::collections::{HashMap, BinaryHeap};
use std::error::Error;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::ops::{Deref, DerefMut};
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Clone, Debug)]
struct Node {
    distance: usize,          
    neighbors: HashMap<usize, usize>  //  연결된 node index, weight
}

impl Node {
    fn new() -> Self {
        Self {  
            distance: usize::MAX,     // start 지점으로부터 연결되어 있지 않은 경우 허용 최대값으로 설정
            neighbors: HashMap::<usize, usize>::new(),
        }
    }

    /// 문제에서와 같이 초기에 주어지는 노드간 연결 edge (방향 O) 를 통해 
    /// node 관 연결 관계 (neibhbors) 생성
    /// 만약 동일 index 이웃이 이미 존재하는 경우, weight 를 변경
    fn add_neighbor(&mut self, idx: usize, weight: usize) {
        self.neighbors
            .entry(idx)
            .and_modify(|w| {
                if *w > weight {
                    *w = weight;
                } else {
                    *w = *w;
                }
            })
            .or_insert(weight);
    }
}

struct Graph(Vec<Node>);

impl Graph {

    /// 문제에서 주어지는 길이 n 에 따라,
    /// 0 ~ n 까지의 vec 생성 (idx 0 은 빈 node 로 사용 X)
    fn new(length: usize) -> Self {
        Self(vec![Node::new(); length + 1])
    }

    /// 문제의 답을 stdout 할 수 있도록 전체 node 의 계산된 distance 를 출력
    fn print(&self) -> Result<(), Box<dyn Error>>{
        let mut output = String::new();
        
        for node in self.0.iter().skip(1) {
            match node.distance {
                usize::MAX => writeln!(output, "INF")?,
                d => writeln!(output, "{}", d)?,
            }
        }
        println!("{}", output);

        Ok(())
    }

    /// 다익스트라 알고리즘을 통해 출발 node 로 부터 각 노드간 최단 distance 를 계산
    fn dijkstra(&mut self, start: usize) {
        let mut priority_queue = BinaryHeap::<QueueItem>::new();

        self.0[start].distance = 0;
        priority_queue.push(QueueItem(start, 0));

        while let Some(QueueItem(idx, dist)) = priority_queue.pop() {
            for (next, weight) in self.0[idx].neighbors.clone().drain() {
                let sum = dist + weight;
                
                if sum < self.0[next].distance {
                    self.0[next].distance = sum;
                    priority_queue.push(QueueItem(next, sum));
                }       
            }
        }
    }
}

impl Deref for Graph {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Eq, PartialEq, Debug)]
struct QueueItem(usize, usize);

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ =  stdin().lock().read_to_string(&mut buf)?;
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap().parse::<usize>();

    let (n, e, start) = (get_num()?, get_num()?, get_num()?);

    let mut graph = Graph::new(n);

    for _ in 0..e {
        let (from, to, weight) = (get_num()?, get_num()?, get_num()?);
        (*graph)[from].add_neighbor(to, weight);
    }

    graph.dijkstra(start);
    graph.print()?;

    Ok(())
}
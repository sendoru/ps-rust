use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use std::{cmp, io, ptr::read, str::FromStr};

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

// forest is given
fn get_parents(adj_list: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n = adj_list.len();
    let mut parents = vec![-1; n];
    let mut visited = vec![false; n];
    fn get_parents_internal(
        adj_list: &Vec<Vec<(usize, i64)>>,
        parents: &mut Vec<i64>,
        visited: &mut Vec<bool>,
        node: usize,
    ) {
        visited[node] = true;
        for &(child, _) in adj_list[node].iter() {
            if visited[child] {
                continue;
            }
            parents[child] = node as i64;
            get_parents_internal(adj_list, parents, visited, child);
        }
    }
    for i in 0..n {
        if !visited[i] {
            get_parents_internal(adj_list, &mut parents, &mut visited, i);
        }
    }
    parents
}

// forest is given
fn get_diameter_and_radius(adj_list: &Vec<Vec<(usize, i64)>>, root: usize) -> (i64, i64) {
    let n = adj_list.len();
    fn bfs(
        adj_list: &Vec<Vec<(usize, i64)>>,
        root: usize,
    ) -> (i64, usize, BTreeMap<usize, i64>, BTreeMap<usize, usize>) {
        let n = adj_list.len();
        let mut dist = BTreeMap::new();
        let mut prev = BTreeMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        dist.insert(root, 0);
        prev.insert(root, root);
        let mut max_dist = 0;
        let mut max_node = root;
        while let Some(node) = queue.pop_front() {
            for &(child, w) in adj_list[node].iter() {
                if dist.contains_key(&child) {
                    continue;
                }
                dist.insert(child, dist[&node] + w);
                prev.insert(child, node);
                if max_dist < dist[&child] {
                    max_dist = dist[&child];
                    max_node = child;
                }
                queue.push_back(child);
            }
        }
        (max_dist, max_node, dist, prev)
    };
    let (_, here, _, _) = bfs(adj_list, root);
    let (diameter, there, dist, prev) = bfs(adj_list, here);
    let mut radius = diameter;
    // dist from here -> node
    let mut cur_dist = diameter;
    let mut node = there;
    while node != here {
        let parent = prev[&node];
        cur_dist = dist[&parent];
        radius = cmp::min(radius, cmp::max(cur_dist, diameter - cur_dist));
        node = parent;
    }
    (diameter, radius)
}

fn get_directed_adj_list(adj_list: &Vec<Vec<(usize, i64)>>, root: usize) -> Vec<Vec<(usize, i64)>> {
    let n = adj_list.len();
    let mut directed_adj_list = vec![vec![]; n];
    let mut visited = vec![false; n];
    fn get_directed_adj_list_internal(
        adj_list: &Vec<Vec<(usize, i64)>>,
        directed_adj_list: &mut Vec<Vec<(usize, i64)>>,
        visited: &mut Vec<bool>,
        node: usize,
    ) {
        visited[node] = true;
        for &(child, w) in adj_list[node].iter() {
            if visited[child] {
                continue;
            }
            directed_adj_list[node].push((child, w));
            get_directed_adj_list_internal(adj_list, directed_adj_list, visited, child);
        }
    }
    for i in 0..n {
        if !visited[i] {
            get_directed_adj_list_internal(adj_list, &mut directed_adj_list, &mut visited, i);
        }
    }
    directed_adj_list
}

pub(crate) fn boj_8872() {
    let nml = read_vec::<i64>();
    let n = nml[0];
    let m = nml[1];
    let l = nml[2];

    let mut adj_list = vec![vec![]; n as usize];
    for _ in 0..m {
        let uvw = read_vec::<i64>();
        let u = uvw[0] as usize;
        let v = uvw[1] as usize;
        let w = uvw[2];
        adj_list[u].push((v, w));
        adj_list[v].push((u, w));
    }

    let adj_list_directed = get_directed_adj_list(&adj_list, 0);
    let parents = get_parents(&adj_list_directed);
    let mut diameters_radii = vec![];
    for i in 0..n as usize {
        if parents[i] == -1 {
            diameters_radii.push(get_diameter_and_radius(&adj_list, i));
        }
    }

    if diameters_radii.len() == 1 {
        println!("{}", diameters_radii[0].0);
        return;
    }

    diameters_radii.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
    let mut largest_component = diameters_radii.pop().unwrap();
    // diameter, radius
    // radius 작은 것부터
    for (diameter, radius) in diameters_radii {
        let new_radius = cmp::max(
            cmp::min(largest_component.1, radius) + l,
            cmp::max(largest_component.1, radius),
        );
        let new_diameter = cmp::max(
            cmp::max(largest_component.0, diameter),
            largest_component.1 + radius + l,
        );
        largest_component = (new_diameter, new_radius);
    }

    println!("{}", largest_component.0);
}

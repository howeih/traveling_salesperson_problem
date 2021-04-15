use std::i32;

static N: i32 = 4;
static COMPLETE_VISIT: i32 = (1 << N) - 1;

fn tsp(dist: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, mark: usize, position: usize) -> i32 {
    if mark == COMPLETE_VISIT as usize {
        return dist[position as usize][0];
    }

    if dp[mark][position] != -1 {
        return dp[mark][position];
    }
    let mut answer = i32::MAX;
    for city in 0..N as usize {
        if (mark & (1 << city)) == 0 {
            let new_answer = dist[position][city] + tsp(dist, dp, mark | (1 << city), city);
            answer = answer.min(new_answer);
        }
    }
    dp[mark][position] = answer;
    return dp[mark][position];
}

fn main() {
    let graph =
        vec![
            vec![0, 10, 15, 20],
            vec![10, 0, 35, 25],
            vec![15, 35, 0, 30],
            vec![20, 25, 30, 0]
        ];

    let mut dp = vec![vec![-1; N as usize]; 1 << N];
    let res = tsp(&graph, &mut dp, 1, 0);
    assert_eq!(res, 80);
}
use std::collections::HashMap;

/// https://leetcode.com/problems/find-players-with-zero-or-one-losses/description/

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Map<player_id, (wins, losses)>
    let mut player_map: HashMap<i32, (i32, i32)> = HashMap::new();

    if matches.is_empty() {
        return vec![];
    }

    // build the player map
    for game in matches {
        let winner = game[0];
        let loser = game[1];

        // if the winner was already in the map, add one to his winning stats
        // else add it to the map with 1 win and 0 losses
        if player_map.contains_key(&winner) {
            let winner_stats = player_map.get_mut(&winner).unwrap();
            winner_stats.0 += 1;
        } else {
            player_map.insert(winner, (1, 0));
        }

        // if the loser was already in the map, add one to his winning stats
        // else add it to the map with 1 loss and 0 wins
        if player_map.contains_key(&loser) {
            let loser_stats = player_map.get_mut(&loser).unwrap();
            loser_stats.1 += 1;
        } else {
            player_map.insert(loser, (0, 1));
        }
    }

    let mut res: Vec<Vec<i32>> = Vec::new();

    // res[0] those that have only wins
    res.push(vec![]);

    // res[1] those that have only one loss
    res.push(vec![]);

    for (player, stats) in player_map {
        if stats.1 == 0 {
            res[0].push(player);
        } else if stats.1 == 1 {
            res[1].push(player);
        }
    }

    res[0].sort();
    res[1].sort();

    res
}

#[cfg(test)]
pub mod tests_find_winners {

    use super::*;

    #[test]
    pub fn test_leetcode1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];

        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];

        let result = find_winners(matches);
        assert_eq!(expected, result);
    }

    #[test]
    pub fn test_leetcode2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];

        let expected = vec![vec![1, 2, 5, 6], vec![]];

        let result = find_winners(matches);
        assert_eq!(expected, result);
    }
}

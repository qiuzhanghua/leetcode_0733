struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn dfs(image: &mut Vec<Vec<i32>>, r: i32, c: i32, color: i32, new_color: i32) {
            if image[r as usize][c as usize] == color {
                image[r as usize][c as usize] = new_color;
                if r >= 1 {
                    dfs(image, r - 1, c, color, new_color);
                }
                if c >= 1 {
                    dfs(image, r, c - 1, color, new_color);
                }
                if r + 1 < image.len() as i32 {
                    dfs(image, r + 1, c, color, new_color);
                }
                if c + 1 < image[0].len() as i32 {
                    dfs(image, r, c + 1, color, new_color);
                }
            }
        }

        let color = image[sr as usize][sc as usize];
        if color != new_color {
            dfs(&mut image, sr, sc, color, new_color);
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }
}

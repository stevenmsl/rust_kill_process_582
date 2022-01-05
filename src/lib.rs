use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
   - you want to kill the process along
     with all its child processes
   - so you need a data structure that
     can store the process id and all
     it child process ids
     - HashMap<usize, Vec<usize>>
*/

impl Solution {
  pub fn kill_process(process: usize, pid: &Vec<usize>, ppid: &Vec<usize>) -> Vec<usize> {
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..pid.len() {
      if !children.contains_key(&ppid[i]) {
        children.insert(ppid[i], vec![]);
        children.get_mut(&ppid[i]).unwrap().push(pid[i]);
      }
    }

    let mut processes = vec![process];
    Self::visit_children(process, &children, &mut processes);
    processes
  }

  fn visit_children(
    process: usize,
    children: &HashMap<usize, Vec<usize>>,
    processes: &mut Vec<usize>,
  ) {
    /* leaf - add it to the processes vec  */
    if !children.contains_key(&process) {
      processes.push(process);
      return;
    }
    let my_children = children.get(&process).unwrap();

    for child in my_children {
      Self::visit_children(*child, children, processes)
    }
  }

  pub fn test_fixture_1() -> Vec<Vec<usize>> {
    vec![vec![1, 3, 10, 5], vec![3, 0, 5, 3]]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let input = Solution::test_fixture_1();
    let result = Solution::kill_process(5, &input[0], &input[1]);

    assert_eq!(result, vec![5, 10]);
  }
}

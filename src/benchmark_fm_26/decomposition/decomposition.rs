use std::collections::HashSet;
use itertools::Itertools;
use crate::interactions::syntax::interaction::Interaction;

impl Interaction {

    pub fn nb_partitions_shuffled(&self,max_number_of_partitions:usize) ->usize {



        let lifelines: Vec<usize> = self.get_lifelines().into_iter().collect();


        let partitions = Self::partitions_at_least_half(&lifelines);


        for (ct,_) in partitions.iter().enumerate() {

            if ct>=max_number_of_partitions{
                return ct-1;
            }


        }

        partitions.len()

    }

    pub fn random_decompose_into_two_ints(&self,max_number_of_partitions:usize) -> Vec<Vec<Interaction>> {
        let mut local_interactions = Vec::new();


        let lifelines: Vec<usize> = self.get_lifelines().into_iter().collect();


        let partitions = Self::partitions_at_least_half(&lifelines);


        for (ct,(lifelines_1, lifelines_2)) in partitions.iter().enumerate() {

            if ct>=max_number_of_partitions{
                break;
            }

            let i1 = self.project(&HashSet::from_iter(lifelines_1.clone()));
            let i2 = self.project(&HashSet::from_iter(lifelines_2.clone()));

            local_interactions.push(vec![i1,i2]);
        }

        local_interactions
    }

    pub fn partitions_at_least_half(v: &Vec<usize>) -> Vec<(Vec<usize>, Vec<usize>)> {
        let n = v.len();
        let min_size = n / 2; // floor(L/2)
        let max_size = n - min_size;

        let mut results = Vec::new();

        // Loop through allowed subset sizes
        for k in min_size..=max_size {
            for combo in v.iter().copied().combinations(k) {
                // Mark which elements were selected
                let mut used = vec![false; n];
                for &c in &combo {
                    // mark by index lookup
                    if let Some(pos) = v.iter().position(|&x| x == c) {
                        used[pos] = true;
                    }
                }

                // Construct the complementary subset
                let other: Vec<usize> = v
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if !used[i] { Some(x) } else { None })
                    .collect();

                results.push((combo, other));
            }
        }

        results
    }

}
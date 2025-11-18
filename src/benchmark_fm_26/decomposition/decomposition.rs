use std::collections::HashSet;
use rand::{rng, Rng};
use rand::seq::index::sample;
use rand::seq::SliceRandom;
use crate::interactions::syntax::interaction::Interaction;

impl Interaction {

    pub fn random_decompose(&self, nb_local_models: usize) ->Vec<Interaction> {
        let mut local_interactions = Vec::new();


        let mut lifelines:Vec<usize> = self.get_lifelines().into_iter().collect();
        lifelines.sort();

        let partitions = if nb_local_models==2{
            /*If the number of desired local models is 2, then wu partition by
            doing the following:
            we shuffle the set of lifelines;
            the fist sets takes the fists floor(|lifelines|) elements of lifeline;
            the next set takes the remaining elements;
             */
            Self:: random_partition_two_equal(&lifelines)
        }
        else{
            Self::random_partition_sorted_vec(&lifelines, nb_local_models)
        };



        for partition in partitions{

            local_interactions.push(self.project(&HashSet::from_iter(partition)));
        }

        local_interactions
    }


    //To modify to allow the partitions to not be contiguous
    pub fn random_partition_sorted_vec(vec: &Vec<usize>, chunks: usize) -> Vec<Vec<usize>> {
        assert!(chunks > 0 && chunks <= vec.len());

        let mut rng = rng();

        // Pick `chunks - 1` unique cut points between 1 and vec.len() - 1
        let mut cut_points: Vec<usize> = sample(&mut rng, vec.len() - 1, chunks - 1)
            .iter()
            .map(|i| i + 1)
            .collect();

        cut_points.sort_unstable();

        let mut result = Vec::with_capacity(chunks);
        let mut start = 0;

        for &cut in &cut_points {
            result.push(vec[start..cut].to_vec());
            start = cut;
        }

        result.push(vec[start..].to_vec());

        result
    }


    pub fn random_partition_unconstrained(vec: &Vec<usize>, chunks: usize) -> Vec<Vec<usize>> {
        assert!(chunks > 0 && chunks <= vec.len());

        let mut rng = rand::rng();

        // Initialize `chunks` empty buckets
        let mut result: Vec<Vec<usize>> = vec![Vec::new(); chunks];

        // Step 1: Ensure no bucket is empty by placing one random element into each
        let mut indices: Vec<usize> = (0..vec.len()).collect();
        indices.shuffle(&mut rng);

        for (bucket, &idx) in result.iter_mut().zip(&indices) {
            bucket.push(vec[idx]);
        }

        // Step 2: Assign remaining elements to a random bucket
        for &idx in &indices[chunks..] {
            let bucket_idx = rng.random_range(0..chunks);
            result[bucket_idx].push(vec[idx]);
        }

        result
    }

    pub fn random_partition_two_equal(vec: &Vec<usize>) -> Vec<Vec<usize>> {
        assert!(vec.len() >= 2);

        let mut rng = rand::rng();

        // Make a shuffled copy
        let mut shuffled = vec.clone();
        shuffled.shuffle(&mut rng);

        let mid = vec.len() / 2; // floor(len/2)

        let left = shuffled[..mid].to_vec();
        let right = shuffled[mid..].to_vec();

        vec![left, right]
    }
}
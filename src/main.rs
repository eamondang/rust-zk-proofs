use petgraph::Graph;
use rand::{seq::SliceRandom, thread_rng, Rng};

// Generate a random graph
fn generate_random_graph<R: Rng>(rng: &mut R) -> Graph<(), ()> {
  let num_nodes = rng.gen_range(5..10);
  let mut graph = Graph::<(), ()>::new();

  for _ in 0..num_nodes {
    graph.add_node(());
  }

  // Connect nodes randomly
  let node_indices: Vec<_> = graph.node_indices().collect();
  for node1 in &node_indices {
    for node2 in &node_indices {
      if rng.gen_bool(0.5) {
        graph.add_edge(*node1, *node2, ());
      }
    }
  }

  graph
}

// Generate a random permutation of node indices
fn generate_random_permutation<R: Rng>(rng: &mut R, num_nodes: usize) -> Vec<usize> {
  let mut permutation: Vec<usize> = (0..num_nodes).collect();
  permutation.shuffle(rng);
  permutation
}

// Commitment: Generate a commitment based on the permutation
fn commit_to_isomorphism<R: Rng>(graph1: &Graph<(), ()>, graph2: &Graph<(), ()>, rng: &mut R) -> Vec<usize> {
  let num_nodes = graph1.node_count();
  generate_random_permutation(rng, num_nodes)
}

// Generate a random challenge
fn generate_challenge<R: Rng>(rng: &mut R) -> bool {
  rng.gen()
}

// Generate a response based on the challenge and commitment
fn generate_response(
  graph1: &Graph<(), ()>,
  graph2: &Graph<(), ()>,
  commitment: &[usize],
  challenge: &bool,
) -> Vec<usize> {
  if *challenge {
    commitment.to_vec() // Clones the commitment vector
  } else {
    generate_random_permutation(&mut thread_rng(), graph1.node_count())
  }
}

// Verify the zero-knowledge proof
fn verify(
  graph1: &Graph<(), ()>,
  graph2: &Graph<(), ()>,
  commitment: &[usize],
  challenge: &bool,
  response: &[usize],
) -> bool {
  let expected_response = if *challenge {
    commitment.to_vec() // Clones the commitment vector
  } else {
    generate_random_permutation(&mut thread_rng(), graph1.node_count())
  };

  response == expected_response
}

fn main() {
  let mut rng = rand::thread_rng();

  // Generate random graphs
  let graph1: Graph<(), ()> = generate_random_graph(&mut rng);
  println!("graph 1: {:?}", graph1);
  let graph2: Graph<(), ()> = generate_random_graph(&mut rng);
  println!("graph 2: {:?}", graph2);

  // Zero-knowledge proof protocol
  let commitment = commit_to_isomorphism(&graph1, &graph2, &mut rng);
  println!("commit : {:?}", commitment);
  let challenge = generate_challenge(&mut rng);
  println!("challenger : {:?}", challenge);
  let response = generate_response(&graph1, &graph2, &commitment, &challenge);
  println!("response: {:?}", response);

  // Verify the zero-knowledge proof
  let is_valid = verify(&graph1, &graph2, &commitment, &challenge, &response);
  println!("Zero-knowledge proof is valid: {}", is_valid);
}

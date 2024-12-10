// Technically the impl is no_std
use crate::Result;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Node {
    pub frequency: char,
    pub xy: glam::IVec2,
}

impl XY for Node {
    fn xy(&self) -> &glam::IVec2 {
        &self.xy
    }

    fn frequency(&self) -> char {
        self.frequency
    }
}

impl XY for &Node {
    fn xy(&self) -> &glam::IVec2 {
        &self.xy
    }

    fn frequency(&self) -> char {
        self.frequency
    }
}

pub trait XY {
    fn xy(&self) -> &glam::IVec2;
    fn frequency(&self) -> char;
}

pub fn solve_part1_impl(input: &impl DataShape) -> Result<usize> {
    let max_xy = input.max_xy();
    let max_xy = &max_xy?;

    let antinodes = input.resonate_pairs().map(|ab| {
        let (a, b) = ab?;
        // We skip the first node because it's the same as the second node.
        // We only take 1 node because part one only considers the first antinode.
        let forward_locations = anitnode_generator(&a, &b).skip(1).take(1);
        let backward_locations = anitnode_generator(&b, &a).skip(1).take(1);

        // neat little trick to capture max_xy so take_while looks clean
        let on_map = |node: &Node| on_map(node, max_xy);

        let valid_forward_locations = forward_locations.take_while(on_map);
        let valid_backward_locations = backward_locations.take_while(on_map);

        Ok::<_, anyhow::Error>(valid_forward_locations.chain(valid_backward_locations))
    });

    // Because of the inner result, we need to unwrap the inner result manually.
    // There should be a flat_map_results or something similar in the future.
    let mut antinode_positions = HashSet::new();
    for antinode in antinodes {
        let antinode = antinode?;
        antinode_positions.extend(antinode.map(|n| n.xy));
    }
    // antinodes.map(|node| Ok(node?.map(|n| n.xy)).collect::<Result<HashSet<_>>>()?;

    Ok(antinode_positions.len())
}

/// Generates antinodes for a given pair of nodes in the direction of a->b
///
/// Given input that looks like `a - b`,
/// it will generate * nodes `a - * - * - *....`
/// where * are the new nodes geneated and includes node b.
///
/// If you want it in the other direction, call it with `b` and `a` instead.
pub fn anitnode_generator(a: &impl XY, b: &impl XY) -> impl Iterator<Item = Node> {
    let diff = b.xy() - a.xy();
    let xy = *b.xy();
    let frequency = b.frequency();
    (0..)
        // compute the new xy
        .map(move |i| xy + diff * i)
        // create the new node
        .map(move |xy| Node { frequency, xy })
}

/// Checks if a node is within the bounds of the map bounds
pub fn on_map(node: &Node, max_xy: &glam::IVec2) -> bool {
    node.xy.x >= 0 && node.xy.y >= 0 && node.xy.x < max_xy.x && node.xy.y < max_xy.y
}

pub fn solve_part2_impl(input: &impl DataShape) -> Result<usize> {
    let max_xy = input.max_xy();
    let max_xy = &max_xy?;

    let antinodes = input.resonate_pairs().map(|ab| {
        let (a, b) = ab?;
        let forward_locations = anitnode_generator(&a, &b);
        let backward_locations = anitnode_generator(&b, &a);

        // neat little trick
        let on_map = |node: &Node| on_map(node, max_xy);

        let valid_forward_locations = forward_locations.take_while(on_map);
        let valid_backward_locations = backward_locations.take_while(on_map);

        Ok::<_, anyhow::Error>(valid_forward_locations.chain(valid_backward_locations))
    });

    let mut antinode_positions = HashSet::new();
    for antinode in antinodes {
        let antinode = antinode?;
        antinode_positions.extend(antinode.map(|n| n.xy));
    }

    Ok(antinode_positions.len())
}

pub trait DataShape {
    type RPNODE<'a>: XY
    where
        Self: 'a;
    fn resonate_pairs(
        &self,
    ) -> impl Iterator<Item = Result<(Self::RPNODE<'_>, Self::RPNODE<'_>)>> + '_;

    fn max_xy(&self) -> Result<glam::IVec2>;
}

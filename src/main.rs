//! Interaction Nets Experiment
//!
//! This is my attempt to understand [interaction nets][paper]. As usual, I'm
//! having trouble understanding the paper itself. I found the [diploma thesis]
//! by Simon Gay very helpful.
//!
//! [paper]: https://dl.acm.org/doi/pdf/10.1145/96709.96718
//! [diploma thesis]: https://www.dcs.gla.ac.uk/~simon/publications/diploma.pdf

#![allow(unused)]

use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
}

/// An interaction net
#[derive(Debug)]
struct Net {
    /// The agents in the interaction net
    agents: &'static [&'static dyn Agent],

    /// The free variables in the interaction net
    variables: &'static [Variable],

    /// Connections between agents in the interaction net
    connections: &'static [(Port, Port)],
}

/// A labeled vertex in a [`Net`]
///
/// An agent has exactly one principal port, identified by `0`. It also has `n`
/// auxiliary ports, identified by `1` to `n`.
trait Agent: Debug {
    /// The label of the agent, called the symbol
    fn symbol(&self) -> &'static str;

    /// The number of auxiliary ports that the agent has
    fn num_aux_ports(&self) -> usize;
}

macro_rules! agents {
    ($($symbol:ident, $num_aux_ports:expr;)*) => {
        $(
            #[derive(Debug)]
            struct $symbol;

            impl Agent for $symbol {
                fn symbol(&self) -> &'static str {
                    stringify!($symbol)
                }

                fn num_aux_ports(&self) -> usize {
                    $num_aux_ports
                }
            }
        )*
    };
}

agents!(
    Add, 2;

    // The unary digit one.
    One, 1;

    // A symbol that terminates a unary number.
    Zero, 0;
);

/// A free variable in a [`Net`]
#[derive(Debug)]
struct Variable {
    /// The name of the variable
    name: &'static str,

    /// The port to which the variable is attached
    ///
    /// If this is `None`, there are no ports, the net is just an edge, and the
    /// variable is attached to that.
    port: Option<Port>,
}

/// A port of an [`Agent`]
#[derive(Debug)]
struct Port {
    /// The agent to which this port belongs
    ///
    /// This is an index into [`Net`]'s `agents` field.
    agent: usize,

    /// The port number
    ///
    /// See [`Agent`] for a description of port numbering.
    number: usize,
}

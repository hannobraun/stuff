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
    // Addition on unary numbers
    let add = Algorithm {
        rules: &[
            RewriteRule {
                left: Net {
                    agents: &[&Add, &Zero],
                    variables: &[
                        Variable {
                            name: "y",
                            port: Some(Port {
                                agent: 0,
                                number: 1,
                            }),
                        },
                        Variable {
                            name: "z",
                            port: Some(Port {
                                agent: 0,
                                number: 2,
                            }),
                        },
                    ],
                    connections: &[(
                        Port {
                            agent: 0,
                            number: 0,
                        },
                        Port {
                            agent: 1,
                            number: 0,
                        },
                    )],
                },
                right: Net {
                    agents: &[],
                    variables: &[
                        Variable {
                            name: "y",
                            port: None,
                        },
                        Variable {
                            name: "z",
                            port: None,
                        },
                    ],
                    connections: &[],
                },
            },
            RewriteRule {
                left: Net {
                    agents: &[&Add, &One],
                    variables: &[
                        Variable {
                            name: "x",
                            port: Some(Port {
                                agent: 1,
                                number: 1,
                            }),
                        },
                        Variable {
                            name: "y",
                            port: Some(Port {
                                agent: 0,
                                number: 1,
                            }),
                        },
                        Variable {
                            name: "z",
                            port: Some(Port {
                                agent: 0,
                                number: 2,
                            }),
                        },
                    ],
                    connections: &[(
                        Port {
                            agent: 0,
                            number: 0,
                        },
                        Port {
                            agent: 1,
                            number: 0,
                        },
                    )],
                },
                right: Net {
                    agents: &[&Add, &One],
                    variables: &[
                        Variable {
                            name: "x",
                            port: Some(Port {
                                agent: 0,
                                number: 0,
                            }),
                        },
                        Variable {
                            name: "y",
                            port: Some(Port {
                                agent: 0,
                                number: 1,
                            }),
                        },
                        Variable {
                            name: "z",
                            port: Some(Port {
                                agent: 1,
                                number: 0,
                            }),
                        },
                    ],
                    connections: &[(
                        Port {
                            agent: 0,
                            number: 2,
                        },
                        Port {
                            agent: 1,
                            number: 1,
                        },
                    )],
                },
            },
        ],
    };

    dbg!(add);
}

/// An algorithm described in the form of rewrite rules on interaction nets
#[derive(Debug)]
struct Algorithm {
    rules: &'static [RewriteRule],
}

/// A rule that rewrites one net to another
#[derive(Debug)]
struct RewriteRule {
    /// The net that is being rewritten
    left: Net,

    /// The result of the rewrite
    right: Net,
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

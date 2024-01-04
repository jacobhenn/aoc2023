mod b;

use aocutil::prelude::*;

pub const YEAR: usize = 2023;

pub const DAY: usize = 19;

struct Part {
    ratings: HashMap<char, u64>,
}

impl Part {
    fn parse(s: &str) -> Self {
        Self {
            ratings: s
                .trim_matches(['{', '}'])
                .split(',')
                .map(|rating| {
                    let mut chars = rating.chars();
                    let category = chars.next().unwrap();
                    chars.next();
                    (category, chars.as_str().parse().unwrap())
                })
                .collect(),
        }
    }
}

enum Filter {
    Comparison {
        category: char,
        ordering: Ordering,
        compare_to: u64,
    },
    Unconditional,
}

impl Filter {
    fn passes(&self, part: &Part) -> bool {
        match self {
            Filter::Comparison {
                category,
                ordering,
                compare_to,
            } => part.ratings[category].cmp(compare_to) == *ordering,
            Filter::Unconditional => true,
        }
    }
}

struct Rule<'a> {
    filter: Filter,
    destination: &'a str,
}

impl<'a> Rule<'a> {
    fn parse(s: &'a str) -> Self {
        let Some((check, destination)) = s.split_once(':') else {
            return Self {
                filter: Filter::Unconditional,
                destination: s,
            };
        };

        let mut check_chars = check.chars();
        let category = check_chars.next().unwrap();
        let ordering = match check_chars.next().unwrap() {
            '<' => Ordering::Less,
            '>' => Ordering::Greater,
            other => panic!("invalid comparison char {other}"),
        };

        let compare_to = check_chars.as_str().parse().unwrap();

        Self {
            filter: Filter::Comparison {
                category,
                ordering,
                compare_to,
            },
            destination,
        }
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn parse(s: &'a str) -> Self {
        Self {
            rules: s.split(',').map(Rule::parse).collect(),
        }
    }

    fn destination(&self, part: &Part) -> &str {
        self.rules
            .iter()
            .find(|rule| rule.filter.passes(part))
            .expect("workflows should have a catch-all")
            .destination
    }
}

fn is_accepted(part: &Part, workflow_name: &str, workflows: &HashMap<&str, Workflow>) -> bool {
    match workflows[workflow_name].destination(part) {
        "A" => true,
        "R" => false,
        next_workflow_name => is_accepted(part, next_workflow_name, workflows),
    }
}

pub fn solve<P: AocPart>(input: &str) -> u64 {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<&str, Workflow> = workflows
        .lines()
        .map(|line| {
            let (name, workflow) = line.split_once('{').unwrap();
            (name, Workflow::parse(workflow.trim_end_matches('}')))
        })
        .collect();

    if P::is_one() {
        parts
            .lines()
            .map(Part::parse)
            .filter(|part| is_accepted(part, "in", &workflows))
            .map(|accepted_part| accepted_part.ratings.values().sum::<u64>())
            .sum()
    } else {
        b::num_accepted(b::PartRange::full(), "in", &workflows)
    }
}

aoc_tests! {
    inputs {
        e0 = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    }

    part::One {
        ea0: e0 => 19114,
        ra: @input => 446517,
    }

    part::Two {
        eb0: e0 => 167409079868000,
        rb: @input => 130090458884662,
    }
}

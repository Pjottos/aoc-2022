use aoc_2022::*;

use dynasmrt::{dynasm, x64::X64Relocation, Assembler, AssemblyOffset, DynasmApi, DynasmLabelApi};
use num::integer::Integer;

use std::{iter, str::FromStr};

fn main() {
    Harness::begin()
        .day(11)
        .extract(|text| {
            text.split("\n\n")
                .map(|block| {
                    let mut parts = block.lines().map(|line| {
                        let (_, v) = line.split_once(':').unwrap();
                        v
                    });

                    parts.next();
                    Monkey {
                        items: parts
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|n| n.trim().parse().unwrap())
                            .collect(),
                        operation: parts.next().and_then(|op| op.parse().ok()).unwrap(),
                        divisible_test: parts
                            .next()
                            .unwrap()
                            .rsplit(' ')
                            .next()
                            .and_then(|n| n.parse().ok())
                            .unwrap(),
                        head: parts
                            .next()
                            .unwrap()
                            .rsplit(' ')
                            .next()
                            .and_then(|n| n.parse().ok())
                            .unwrap(),
                        tail: parts
                            .next()
                            .unwrap()
                            .rsplit(' ')
                            .next()
                            .and_then(|n| n.parse().ok())
                            .unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .run_part(1, |mut monkeys| run(&mut monkeys, 20, false))
        .run_part(2, |mut monkeys| {
            let lcm = monkeys
                .iter()
                .fold(1, |lcm, monkey| lcm.lcm(&(monkey.divisible_test as u64)));
            assert_eq!(lcm, 9699690);
            run(&mut monkeys, 10000, true)
        });
}

fn run(monkeys: &mut [Monkey], rounds: u64, use_lcm: bool) -> u64 {
    assert_ne!(rounds, 0);

    let mut asm = Assembler::<X64Relocation>::new().unwrap();
    let item_count = monkeys.iter().map(|m| m.items.len()).sum::<usize>();
    let mut item_stacks = monkeys
        .iter()
        .flat_map(|monkey| {
            iter::once(monkey.items.len() as u64)
                .chain(monkey.items.iter().copied())
                .chain(iter::repeat(0).take(item_count - monkey.items.len()))
        })
        .collect::<Box<_>>();

    let labels = iter::repeat_with(|| asm.new_dynamic_label())
        .take(2 * monkeys.len())
        .collect::<Box<_>>();
    dynasm!(asm
        ; push r12
        ; push r13
        ; mov rcx, rdx
        // rdi: inspect_counts
        // rsi: item_stacks
        // rcx: rounds
        ; ->round:
    );
    for (i, monkey) in monkeys.iter().enumerate() {
        let local_labels = &labels[2 * i..2 * (i + 1)];
        let stack_offset = (i * (1 + item_count) * 8) as i32;
        let inspect_offset = (i * 8) as i32;
        dynasm!(asm
            ; =>local_labels[0]
            ; mov r9, QWORD [rsi + stack_offset]
            ; mov QWORD [rsi + stack_offset], 0
            ; xor r8, r8
            ; =>local_labels[1]
            ; cmp r8, r9
        );
        // Jump to next monkey, or if this is the last monkey, to the round end
        if i != monkeys.len() - 1 {
            dynasm!(asm; je =>labels[2 * (i + 1)]);
        } else {
            dynasm!(asm; je ->round_end);
        }

        // Load next item and increment inspect count
        dynasm!(asm
            ; mov rax, QWORD [rsi + stack_offset + 8 + r8 * 8]
            ; inc QWORD [rdi + inspect_offset]
        );

        match monkey.operation {
            Operation::AddSelf => dynasm!(asm; lea rax, [rax + rax]),
            Operation::MulSelf => dynasm!(asm; mul rax),
            Operation::Add(n) => dynasm!(asm; lea rax, [rax + i32::from(n)]),
            // TODO: compute equivalent shifts and adds
            Operation::Mul(n) => dynasm!(asm; imul rax, rax, i32::from(n)),
        }

        if use_lcm {
            // % 9699690
            // TODO: compute this instead of hardcoding it
            dynasm!(asm
                ; mov r11, rax
                ; shr rax, 1
                ; mov r10, 7976672703492201007
                ; mul r10
                ; shr rdx, 21
                ; imul rax, rdx, 9699690
                ; sub r11, rax
            );
        } else {
            // / 3
            dynasm!(asm
                ; mov r10, -6148914691236517205
                ; mul r10
                ; mov r11, rdx
                ; shr r11, 1
            );
        }

        assert!(monkey.head != i && monkey.head < monkeys.len());
        assert!(monkey.tail != i && monkey.tail < monkeys.len());
        let head_stack_offset = (monkey.head * (item_count + 1) * 8) as i32;
        let tail_stack_offset = (monkey.tail * (item_count + 1) * 8) as i32;
        dynasm!(asm
            ; lea r12, [rsi + head_stack_offset]
            ; lea r13, [rsi + tail_stack_offset]
        );
        // TODO: compute this instead of hardcoding it
        if monkey.divisible_test == 2 {
            dynasm!(asm
                ; test r11b, 1
                ; cmove r13, r12
            );
        } else {
            let (a, b) = match monkey.divisible_test {
                3 => (-6148914691236517205, 6148914691236517206),
                5 => (-3689348814741910323, 3689348814741910324),
                7 => (7905747460161236407, 2635249153387078803),
                11 => (3353953467947191203, 1676976733973595602),
                13 => (5675921253449092805, 1418980313362273202),
                17 => (-1085102592571150095, 1085102592571150096),
                19 => (-8737931403336103397, 970881267037344822),
                _ => todo!("compute constant modulo"),
            };
            dynasm!(asm
                ; mov rax, QWORD a
                ; imul rax, r11
                ; mov rdx, QWORD b
                ; cmp rax, rdx
                ; cmovb r13, r12
            );
        }

        dynasm!(asm
            ; mov rdx, QWORD [r13]
            ; inc QWORD [r13]
            ; mov QWORD [r13 + 8 + rdx * 8], r11
            ; inc r8b
            ; jmp =>local_labels[1]
        );
    }
    dynasm!(asm
        ; ->round_end:
        ; dec rcx
        ; jne ->round
        ; pop r13
        ; pop r12
        ; ret
    );

    let exec_buffer = asm.finalize().unwrap();
    //println!("{:02X?}", &exec_buffer[..]);
    let mut inspect_counts = vec![0; monkeys.len()];
    unsafe {
        let func: unsafe extern "sysv64" fn(*mut u64, *mut u64, u64) =
            std::mem::transmute(exec_buffer.ptr(AssemblyOffset(0)));
        func(
            inspect_counts.as_mut_ptr(),
            item_stacks.as_mut_ptr(),
            rounds,
        );
    }

    inspect_counts.sort_unstable();
    inspect_counts.iter().rev().take(2).product::<u64>()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    AddSelf,
    MulSelf,
    Add(u8),
    Mul(u8),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, op) = s.split_once("new = old ").ok_or(())?;
        let res = match op {
            "+ old" => Self::AddSelf,
            "* old" => Self::MulSelf,
            _ if op.starts_with('+') => Self::Add(
                op.split(' ')
                    .nth(1)
                    .and_then(|n| n.parse().ok())
                    .ok_or(())?,
            ),
            _ if op.starts_with('*') => Self::Mul(
                op.split(' ')
                    .nth(1)
                    .and_then(|n| n.parse().ok())
                    .ok_or(())?,
            ),
            _ => return Err(()),
        };
        Ok(res)
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_test: u8,
    head: usize,
    tail: usize,
}

fn read_memory() -> Vec<usize> {
    include_str!("../input")
        .trim()
        .split(',')
        .map(|x| {
            x.parse()
                .ok()
                .unwrap_or_else(|| panic!("Failed to parse input {}!", &x))
        })
        .collect()
}

fn run(mut mem: Vec<usize>) -> Option<(usize, usize)> {
    let mut pc = 0;
    loop {
        let instruction = mem[pc];
        let addr1 = mem.get(pc + 1)?;
        let addr2 = mem.get(pc + 2)?;
        let target = *mem.get(pc + 3)?;

        if target >= mem.len() {
            return None;
        }

        mem[target] = match instruction {
            1 => mem.get(*addr1)? + mem.get(*addr2)?,
            2 => mem.get(*addr1)? * mem.get(*addr2)?,

            99 => return Some((mem[0], pc)),
            _ => panic!("Invalid opcode {}", instruction),
        };

        pc += 4;
    }
}

fn main() {
    let mem = read_memory();

    for n in 0..99 {
        println!("Running noun {} ...", n);
        for v in 0..99 {
            let mut run_mem = mem.clone();
            run_mem[1] = n;
            run_mem[2] = v;

            let res = run(run_mem);

            if let Some((value, _)) = res {
                if value == 19_690_720 {
                    println!("Noun: {} Verb: {}, Result: {}", n, v, 100 * n + v);
                    return;
                }
            }
        }
    }
}

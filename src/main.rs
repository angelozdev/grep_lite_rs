static TEXT: &str = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

fn main() {
    let needle = "is";
    let mut tags: Vec<usize> = vec![];
    let ctx_lines = 1;
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in TEXT.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i)
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in TEXT.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let lines_as_string = String::from(line);
                let local_ctx = (i, lines_as_string);

                match ctx.get_mut(j) {
                    Some(c) => c.push(local_ctx),
                    None => ctx.push(vec![local_ctx]),
                }
            }
        }
    }

    for local_ctx in ctx.iter() {
        for (i, line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}. {}", line_num, line);
        }
        println!("\n");
    }
}

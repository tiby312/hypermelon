use hypermelon::build;
use hypermelon::prelude::*;
fn main() -> std::fmt::Result {
    let html = build::single("DOCTYPE html")
        .with_start("!")
        .with_ending("")
        .chain(build::elem("html"));

    let style = build::elem("style").append(
        "table, th, td {
border: 1px solid black;
border-collapse: collapse;
animation: mymove 5s infinite;
}
@keyframes mymove {
    from {background-color: red;}
    to {background-color: blue;}
}",
    );

    let table = {
        let table = build::elem("table").with(("style", format_move!("width:{}%", 100)));

        let rows = (0..20).map(|i| {
            build::from_closure(move |w| {
                if i % 2 == 0 {
                    let columns = elems!(
                        build::elem("th")
                            .inline()
                            .append(format_move!("Hay {}:1", i)),
                        build::elem("th")
                            .inline()
                            .append(format_move!("Hay {}:2", i)),
                        build::elem("th")
                            .inline()
                            .append(format_move!("Hay {}:3", i))
                    );

                    w.render(build::elem("tr").inline().append(columns))?;
                } else {
                    let column = build::elem("th")
                        .inline()
                        .append(format_move!("Hay {}:1", i));
                    w.render(build::elem("tr").inline().append(column))?;
                }
                Ok(())
            })
        });
        table.append(build::from_iter(rows))
    };

    let all = html.append(style).append(table);

    use hypermelon::render::*;
    Renderer::new()
        .with_fmt(PrettyFmt::new().with_tab(" "))
        .render(all, hypermelon::stdout_fmt())
}

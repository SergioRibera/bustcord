use egui::{Align2, Area, Context, Id, Widget};

pub fn home(
    ctx: &Context,
    servers: impl Widget,
    chats: impl Widget,
    content: impl Widget,
    opt: Option<impl Widget>,
) {
    Area::new(Id::new("main"))
        .anchor(Align2::LEFT_TOP, (0., 0.))
        .interactable(false)
        .fade_in(true)
        .movable(false)
        .show(ctx, |ui| {
            ui.columns(3 + opt.as_ref().map(|_| 1).unwrap_or_default(), |cols| {
                cols[0].set_min_width(40.0);
                cols[0].add(servers);
                cols[0].separator();

                cols[1].set_min_width(100.0);
                cols[1].add(chats);
                cols[1].separator();

                cols[2].set_min_width(300.0);
                cols[2].add(content);

                if let Some(opt) = opt {
                    cols[2].separator();
                    cols[3].set_width(100.0);
                    _ = cols[3].add(opt);
                }
            });
        });
}

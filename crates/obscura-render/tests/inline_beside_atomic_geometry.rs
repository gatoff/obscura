//! An ordinary inline element that shares a line with an atomic inline
//! (input, button, image, inline-block) must still get geometry. It used to
//! have no rect at all, so getBoundingClientRect() returned zeros and
//! automation tools treated links next to buttons as invisible.
use obscura_dom::tree_sink::parse_html;
use obscura_render::layout_dom;

fn rect_of(html: &str) -> (obscura_render::Rect, obscura_render::Rect) {
    let tree = parse_html(html);
    let laid = layout_dom(&tree, (800.0, 600.0));
    let t = tree.get_element_by_id("t").expect("t");
    let x = tree.get_element_by_id("x").expect("x");
    (*laid.rects.get(&t).expect("inline element has a rect"), laid.rects[&x])
}

#[test]
fn inline_link_beside_atomic_inlines_has_geometry() {
    for sibling in [
        "<input id=x>",
        "<button id=x>b</button>",
        "<img id=x width=10 height=10>",
        "<span id=x style='display:inline-block'>x</span>",
    ] {
        let (t, x) = rect_of(&format!("<!doctype html><div><a id=t href=#>Try</a>{sibling}</div>"));
        assert!(t.width > 0.0 && t.height > 0.0, "{sibling}: {t:?}");
        assert!(t.x + t.width <= x.x + 0.5, "{sibling}: link must end where the sibling starts: {t:?} {x:?}");
    }
}

#[test]
fn nested_inlines_beside_an_atomic_inline_resolve_inside_out() {
    let tree = parse_html("<!doctype html><div><span id=outer>Hello <b id=t>bold</b></span><input id=x></div>");
    let laid = layout_dom(&tree, (800.0, 600.0));
    let outer = laid.rects[&tree.get_element_by_id("outer").unwrap()];
    let inner = laid.rects[&tree.get_element_by_id("t").unwrap()];
    assert!(inner.width > 0.0 && outer.width > inner.width, "{outer:?} {inner:?}");
    assert!(outer.x <= inner.x && outer.x + outer.width >= inner.x + inner.width - 0.5);
}

pub fn common(page: String) -> String {
    let page = page.replace("$$FOOTER$$", include_str!("../html/footer"));
    let page = page.replace("$$HEAD$$", include_str!("../html/head"));

    page
}

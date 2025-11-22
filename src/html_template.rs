pub static APP_CSS: &str = include_str!("../static/app.css");
pub static SHINDAN_JS: &str = include_str!("../static/shindan.js");
pub static APP_JS: &str = include_str!("../static/app.js");
pub static CHART_JS: &str = include_str!("../static/chart.js");

pub fn build_html() -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="zh" style="height:100%">
<head>
    <!-- BASE_URL -->
    <style>
        {}
    </style>
    <meta http-equiv="Content-Type" content="text/html;charset=utf-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0,minimum-scale=1.0">

    <title>ShindanMaker</title>
</head>
<body class="" style="position:relative;min-height:100%;top:0">
    <div id="main-container">
        <div id="main"><!-- TITLE_AND_RESULT --></div>
    </div>
</body>

<script>
    {}
</script>

<!-- SCRIPTS -->
</html>"#,
        APP_CSS, SHINDAN_JS
    )
}

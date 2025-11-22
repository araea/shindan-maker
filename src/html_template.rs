pub const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="zh" style="height:100%">
<head>
    <!-- BASE_URL -->
    <link rel="stylesheet" href="/css/app.css">
    <meta http-equiv="Content-Type" content="text/html;charset=utf-8">
    <meta name="viewport" content="width=device-width,initial-scale=1.0,minimum-scale=1.0">
    <script src="/js/shindan.js" defer></script>
    <!-- SCRIPTS -->
    <title>ShindanMaker</title>
</head>
<body class="" style="position:relative;min-height:100%;top:0">
    <div id="main-container">
        <div id="main"><!-- TITLE_AND_RESULT --></div>
    </div>
</body>
</html>"#;

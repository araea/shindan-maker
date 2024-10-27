pub const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>

<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width">
    <link rel="stylesheet" href="https://cn.shindanmaker.com/css/app.css?id=cbfb28ec9001aee269676b04e227a3b9">
    <style>
        :root {
            --body-bg: #ffffff;
            --text-body: #212529;
            --bg-img-line: #ffffff;
            --bg-img-fill: #ffffff;
            --main-blue: #00c5ff;
        }

        html {
            box-sizing: border-box;
            font-family: sans-serif;
            line-height: 1.15;
            -webkit-tap-highlight-color: transparent;
            max-width: 750px;
        }

        *, *::before, *::after {
            box-sizing: inherit;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            font-size: 0.9rem;
            font-weight: 400;
            line-height: 1.6;
            color: var(--text-body);
            background-color: var(--body-bg);
            background-image: repeating-linear-gradient(90deg, var(--bg-img-line) 1px, transparent 1px, transparent 15px),
            repeating-linear-gradient(0deg, var(--bg-img-line) 1px, var(--bg-img-fill) 1px, var(--bg-img-fill) 15px);
            background-size: 15px 15px;
            margin: 0;
            text-align: left;
            overflow-wrap: break-word;
            max-width: 750px;
            height: 100%;
        }

        #main-container {
            max-width: 750px;
        }

        #main {
            min-height: 500px;
        }

        #title_and_result {
            width: 100%;
            margin-bottom: 1rem;
            border: 1rem solid var(--main-blue);
            font-size: 1.9rem;
        }

        #shindanResultAbove {
            padding: 1.5rem;
            text-align: center;
            font-weight: 700;
            font-size: 1.1em;
            line-height: 1.2;
        }

        #shindanResultAbove span {
            display: inline-block;
            text-align: left;
        }

        #shindanResultAbove a {
            font-weight: 700;
            text-decoration: none;
            color: var(--text-body);
        }


        #shindanResultTitle {
            display: block;
            overflow: hidden;
            padding: 1.5rem 0.5rem;
            white-space: nowrap;
            text-align: center;
            font-weight: 700;
            background-color: var(--main-blue);
            color: #fff;
            line-height: 1.1em;
            font-size: 0.9em;
        }

        #shindanResultContainer {
            font-size: 1em;
        }

        #shindanResultHeight {
            display: flex;
            min-height: 200px;
            width: 100%;
            align-items: center;
        }

        #shindanResultCell {
            width: 100%;
        }

        #shindanResultContent {
            display: block;
            padding: 1.5rem;
            text-align: center;
            word-break: break-word;
        }

        #shindanResult {
            display: inline-block;
            text-align: left;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            line-height: 1.33;
            white-space: break-spaces;
        }

        #shindanResult span {
            font-weight: 700;
        }

        #title_and_result > .shindanTitleImageContainer {
            font-family: -apple-system, BlinkMacSystemFont, "Helvetica Neue", Arial, "Hiragino Kaku Gothic ProN", "Hiragino Sans", Meiryo, sans-serif;
            color: var(--text-body);
            font-size: 1.9rem;
            aspect-ratio: 40/21;
            width: 100%;
        }

        #title_and_result > .shindanTitleImageContainer > a {
            font-weight: 700;
            color: #fff !important;
            text-decoration: none !important;
        }

        #title_and_result > .shindanTitleImageContainer > a > img {
            width: 100%;
            height: auto;
            display: block;
            max-width: 960px;
        }
    </style>
    <!-- SCRIPTS -->
    <title>ShindanMaker</title>
</head>

<body>
<div id="main-container">
    <div id="main">
        <!-- TITLE_AND_RESULT -->
    </div>
</div>
</body>
</html>"#;
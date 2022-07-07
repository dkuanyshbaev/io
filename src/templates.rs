pub const HOME: &str = r#"
<html>
    <head>
        <title>IOracle</title>
    </head>
    <body>
        <h4>Welcome to IOracle!</h4>
        <form action="/question" method="post">
            <label for="question">Question:</label>
            <input id="question" type="text" name="question">
            <input type="submit" value="Ask">
        </form>
    </body>
</html>
"#;

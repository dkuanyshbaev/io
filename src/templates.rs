pub const HOME: &str = r#"
<html>
    <head>
        <title>IOracle</title>
    </head>
    <body>
        <h4>Welcome to IOracle!</h4>
        <form action="/question" method="post">
            <p><textarea rows="8" cols="42" name="question"></textarea></p>
            <input type="submit" value="Ask">
        </form>
    </body>
</html>
"#;

pub const ANSWER: &str = r#"
<html>
    <head>
        <title>IOracle</title>
    </head>
    <body>
        <h4>Answer is 42!</h4>
        <a href="/">Back</a>
    </body>
</html>
"#;

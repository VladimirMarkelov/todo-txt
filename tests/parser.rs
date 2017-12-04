extern crate todo_txt;

#[test]
fn simple_task()
{
    let line = "Email SoAndSo at soandso@example.com\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn finished()
{
    let line = "x done\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "done".to_owned(),
        finished: true,
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn created()
{
    let line = "x 2017-11-25 subject\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "subject".to_owned(),
        create_date: Some(::todo_txt::Date::from_ymd(2017, 11, 25)),
        finished: true,
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn completed()
{
    let line = "x 2017-11-26 2017-11-25 subject\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "subject".to_owned(),
        create_date: Some(::todo_txt::Date::from_ymd(2017, 11, 25)),
        finish_date: Some(::todo_txt::Date::from_ymd(2017, 11, 26)),
        finished: true,
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn priority()
{
    let line = "x (A) 2017-11-26 2017-11-25 subject\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "subject".to_owned(),
        priority: 0,
        create_date: Some(::todo_txt::Date::from_ymd(2017, 11, 25)),
        finish_date: Some(::todo_txt::Date::from_ymd(2017, 11, 26)),
        finished: true,
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn contexts()
{
    let line = "Email SoAndSo at soandso@example.com @context1 @context2\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com @context1 @context2".to_owned(),
        contexts: vec!["context1".to_owned(), "context2".to_owned()],
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn deplucate_contexts()
{
    let line = "Email SoAndSo at soandso@example.com @context1 @context2 @context1\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com @context1 @context2 @context1".to_owned(),
        contexts: vec!["context1".to_owned(), "context2".to_owned()],
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn projects()
{
    let line = "Email SoAndSo at soandso@example.com +project1 @context2\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com +project1 @context2".to_owned(),
        contexts: vec!["context2".to_owned()],
        projects: vec!["project1".to_owned()],
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn hashtags()
{
    let line = "Email SoAndSo at soandso@example.com +project1 #tag @context2\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com +project1 #tag @context2".to_owned(),
        contexts: vec!["context2".to_owned()],
        projects: vec!["project1".to_owned()],
        hashtags: vec!["tag".to_owned()],
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn tags()
{
    use ::std::collections::BTreeMap;

    let mut tags = BTreeMap::new();
    tags.insert("key1".to_owned(), "2018-01-01".to_owned());
    tags.insert("key2".to_owned(), "value".to_owned());

    let line = "Email SoAndSo at soandso@example.com key1:2018-01-01 key2:value\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        tags: tags,
        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn due()
{
    let line = "Email SoAndSo at soandso@example.com due:2018-01-01\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        due_date: Some(::todo_txt::Date::from_ymd(2018, 1, 1)),

        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}

#[test]
fn threshold()
{
    let line = "Email SoAndSo at soandso@example.com t:2018-01-01\n".to_owned();
    let task = ::todo_txt::Task {
        subject: "Email SoAndSo at soandso@example.com".to_owned(),
        threshold_date: Some(::todo_txt::Date::from_ymd(2018, 1, 1)),

        .. Default::default()
    };

    assert_eq!(::todo_txt::parser::task(&line), task);
}
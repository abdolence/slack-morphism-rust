# Contributing

Welcome! Please read this document to understand what you can do:
* [Analyze Issues](#analyze-issues)
* [Report an Issue](#report-an-issue)
* [Contribute Code](#contribute-code)

## Analyze Issues

Analyzing issue reports can be a lot of effort. Any help is welcome!
Go to the GitHub issue tracker and find an open issue which needs additional work or a bugfix (e.g. issues labeled with "help wanted" or "bug").
Additional work could include any further information, or a gist, or it might be a hint that helps understanding the issue.

## Report an Issue

If you find a bug - you are welcome to report it.
You can go to the GitHub issue tracker to report the issue.

### Quick Checklist for Bug Reports

Issue report checklist:
* Real, current bug for the latest/supported version
* No duplicate
* Reproducible
* Minimal example

### Issue handling process

When an issue is reported, a committer will look at it and either confirm it as a real issue, close it if it is not an issue, or ask for more details.
An issue that is about a real bug is closed as soon as the fix is committed.


### Reporting Security Issues

If you find or suspect a security issue, please act responsibly and do not report it in the public issue tracker, but directly to us, so we can fix it before it can be exploited.
For details please check our [Security policy](SECURITY.md).

## Contribute Code

You are welcome to contribute code in order to fix bugs or to implement new features.

There are three important things to know:

1.  You must be aware of the Apache License (which describes contributions) and **agree to the Contributors License Agreement**. This is common practice in all major Open Source projects.
    For company contributors special rules apply. See the respective section below for details.
2.  **Not all proposed contributions can be accepted**. Some features may e.g. just fit a third-party add-on better. The code must fit the overall direction and really improve it. The more effort you invest, the better you should clarify in advance whether the contribution fits: the best way would be to just open an issue to discuss the feature you plan to implement (make it clear you intend to contribute).

### Contributor License Agreement

When you contribute (code, documentation, or anything else), you have to be aware that your contribution is covered by the same [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0).

This applies to all contributors, including those contributing on behalf of a company.

### Contribution Content Guidelines

These are some of the rules we try to follow:

-   Apply a clean coding style adapted to the surrounding code, even though we are aware the existing code is not fully clean
-   Use variable naming conventions like in the other files you are seeing
-   No println() - use logging service if needed
-   Comment your code where it gets non-trivial
-   Keep an eye on performance and memory consumption, properly destroy objects when not used anymore
-   Avoid incompatible changes if possible, especially do not modify the name or behavior of public API methods or properties

### How to contribute - the Process

1.  Make sure the change would be welcome (e.g. a bugfix or a useful feature); best do so by proposing it in a GitHub issue
2.  Create a branch forking the cla-assistant repository and do your change
3.  Commit and push your changes on that branch
4.  In the commit message
   - Describe the problem you fix with this change.
   - Describe the effect that this change has from a user's point of view. App crashes and lockups are pretty convincing for example, but not all bugs are that obvious and should be mentioned in the text.
   - Describe the technical details of what you changed. It is important to describe the change in a most understandable way so the reviewer is able to verify that the code is behaving as you intend it to.
5.  Create a Pull Request
6.  Once the change has been approved we will inform you in a comment
7.  We will close the pull request, feel free to delete the now obsolete branch

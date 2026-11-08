# Contributing

This document describes how you can help:

* [Analyse issues](#analyse-issues)
* [Report an issue](#report-an-issue)
* [Contribute code](#contribute-code)

## Analyse issues

Analysing issue reports can take a lot of effort, and any help is welcome.
Go to the GitHub issue tracker and find an open issue that needs additional work or a bug fix (for example, issues labelled "help wanted" or "bug").
Additional work could include further information, a gist, or a hint that helps to understand the issue.

## Report an issue

If you find a bug, you are welcome to report it in the GitHub issue tracker.

### Quick checklist for bug reports

Check that the issue is:

* a real, current bug in the latest or a supported version;
* not a duplicate;
* reproducible;
* described with a minimal example.

### Issue handling process

When an issue is reported, a committer looks at it and either confirms it as a real issue, closes it if it is not an issue, or asks for more details.
An issue about a real bug is closed as soon as the fix is committed.

### Reporting security issues

If you find or suspect a security issue, act responsibly and do not report it in the public issue tracker.
Report it privately as described in the [security policy](SECURITY.md), so that it can be fixed before it is exploited.

## Contribute code

You are welcome to contribute code to fix bugs or to implement new features.

There are two important things to know:

1. Your contribution is licensed under the Apache License 2.0, see [Licence](#licence).
2. **Not all proposed contributions can be accepted**. Some features may fit a third-party add-on better. The code must fit the overall direction of the project and improve it. The more effort you invest, the more important it is to clarify in advance whether the contribution fits: the best way is to open an issue to discuss the feature you plan to implement, and to make clear that you intend to contribute.

### Licence

Every contribution (code, documentation, or anything else) is licensed under the same [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) as the rest of the repository, see `LICENSE`.
This applies to all contributors, including those contributing on behalf of a company.

### Contribution content guidelines

Contributions follow these rules:

- Apply a clean coding style adapted to the surrounding code, even though the existing code is not fully clean;
- Use naming conventions like in the other files;
- Use `tracing` for diagnostic output instead of `println!`;
- Comment your code where it gets non-trivial;
- Keep an eye on performance and memory consumption;
- Avoid incompatible changes if possible, especially do not change the name or behaviour of public API items.

### Contribution process

1. Make sure the change would be welcome (for example a bug fix or a useful feature); the best way is to propose it in a GitHub issue.
2. Fork this repository, create a branch and make your change on it.
3. Commit and push your changes on that branch.
4. In the commit message:
   - Describe the problem the change fixes.
   - Describe the effect the change has from a user's point of view. Crashes and lockups are obvious effects; less obvious ones need describing in the text.
   - Describe the technical details of what changed, in the most understandable way, so that the reviewer can verify that the code behaves as intended.
5. Create a pull request.
6. Once the change has been approved, a committer informs you in a comment.
7. A committer closes the pull request. Feel free to delete the now obsolete branch.

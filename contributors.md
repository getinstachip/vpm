# Contributing Guidelines

First off, thank you for considering contributing to the Verilog Package Manager (VPM). It's people like you that make VPM such a great tool.

Following these guidelines helps to communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect in addressing your issue, assessing changes, and helping you finalize your pull requests.

## Getting started
Anybody is welcome to contribute—we encourage anybody who is interested in this project to join the VPM discord. We'll discuss upcoming changes, user suggesions, and roadmaps.

_For something that is bigger than a one or two line fix:_
1. Create your own fork of the code
2. Do the changes in your fork (be sure to follow the code style of the project)
3. If you like the change and think the project could use it, send a pull request indicating that you have a CLA on file (for these larger fixes, try to include an update on the discord as well)

_For small or "obvious" fixes..._
* Small contributions such as fixing spelling errors, where the content is small enough to not be considered intellectual property, can be submitted by a contributor as a patch

**As a rule of thumb, changes are obvious fixes if they do not introduce any new functionality or creative thinking.** As long as the change does not affect functionality, some likely examples include the following:
- Spelling / grammar fixes
- Typo correction, white space and formatting changes
- Comment clean up
- Bug fixes that change default return values or error codes stored in constants
- Adding logging messages or debugging output
- Changes to ‘metadata’ files like Gemfile, .gitignore, build scripts, etc.
- Moving source files from one directory or package to another

## How to report a bug
**Security Disclosure**
If you find a security vulnerability, do **NOT** open an issue. Email jag.maddipatla@gmail.com or sathvik.redrouthu@gmail.com instead. Any security issues should be submitted here directly.
In order to determine whether you are dealing with a security issue, ask yourself these two questions:
* Can I access something that's not mine, or something I shouldn't have access to?
* Can I disable something for other people?
If the answer to either of those two questions are "yes", then you're probably dealing with a security issue. Note that even if you answer "no" to both questions, you may still be dealing with a security issue, so if you're unsure, just email us.

## Code review process
Once you submit a contribution, it will be signed off by either @Jag-M or @sathvikr prior to being implemented. Interested contributors should join our discord to get commit access.
We also hold weekly triage meetings in a public google meet that all contributors/interested persons may join. Any community feedback will be implemented as soon as possible (usually within a couple of hours).

## Philosophy
Our philosophy is to provide robust tooling to make chip design as intuitive as possible.

If you find yourself wishing for a feature that doesn't exist in VPM, you are probably not alone. There are bound to be others out there with similar needs. Many of the features that VPM has today have been added because our users saw the need. Open an issue on our issues list on GitHub which describes the feature you would like to see, why you need it, and how it should work.

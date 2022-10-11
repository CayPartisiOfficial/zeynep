# something something thank you, something something guidelines

these aren't hard rules but try to stick to them

## commits

0. commits should be reasonably small. ideally with only one change (feature, fix, etc)
1. commits should be formatted like `thing: description of change`
    - you can add more `thing:`s if you want to
    - ideally `description of change` would complete the sentence `This commit will <description>` for the `thing`
    - for example: `emoji: modularize`, `cool-thing-idk: create`, `tooling: prettier: configure formatter`
2. after a single line commit message, you should leave a blank line and go in detail about your change, if it's something that needs that level of explanation

## branches

**for people with write access to the main repo only**. forks can do whatever they want here

0. **ALL NON-TRIVIAL CHANGES MUST BE ON THEIR OWN BRANCHES**
    - urgent fixes can be pushed directly to `main`
1. name your branch like `your-identifier/thing`
    - `your-identifier` can be anything you want, but it should be unique to you & consistent
    - `thing` refers to your change. describe it in a few letters. you can also add more `/`s and details if you _really_ want to
    - for example: `admi/modular`, `admi/emoji/allow-more-per-user`

## merging changes

merges to production _should_ be done via pull requests to the `main` branch. do not merge your own request immediately, wait for code rewiew. `main` should always compile and ideally always run correctly

### checks

make sure `npm run check` succeeds. maybe we'll automate that. maybe we won't. who knows?

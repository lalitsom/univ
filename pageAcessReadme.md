Navbar
    (signIn button or profile if logged in)


All Pages

Anyone over juspay network can open
    - /  (home)
    - login
    - sign up
    - problems (more info using user stats)
    - problem (submit input/button if user logged in, Already solved using user stats)
    - leaderboard (highlight user if logged in)

Check Auth, if failed redirect to login:
    - profile


APIs which can be called from pages
- /api/user/login
- /api/user/logout  (login required with same user)
- /api/user/myStats (login required with same user)
    : list of problems solved, attemptCount
    : email/username/joinDate
- /api/problem/submit (login required with same user)
    : problemId, answer
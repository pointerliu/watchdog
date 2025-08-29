# Fetcher

- get available fetchers, fetcher_types are determined when booting server (now we only support
  ArxivFetcher): ([fetcher_types, ...])
- add new fetcher with a user customized name: (user_id, fetcher_name, fetcher_type)
- remove fetcher by fetcher_name: (user_id, fetcher_name)

# Subscription

- get user's current subscription: (user_id)
- add new subscription (now we only support ArxivCriteria, ArxivCriteria only need a set of keywords): (user_id,
  subscription_name, [keywords, ...])
- remove subscription: (user_id, subscription_name)

# Notifier

- get available notifiers, notifier_types are determined when booting server (now we only support ArxivConsoleNotifier,
  ArxivEmailNotifier): ([notifier_types, ...])
- get user's current notifier: (user_id)
- add new notifier with a user customized name: (user_id, notifier_name, notifier_type), different notifier may need
  extra parameters
    - (extra parameters, ...):
        - console: no extra parameters
        - email: (email_address,)
- remove notifier by notifier_name: (user_id, notifier_name)

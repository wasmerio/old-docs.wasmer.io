# Setup your Ruby environment

To be able to run Wasmer inside our Ruby application, we will need Ruby to be installed in our system.

The easiest way to get [Ruby](https://www.ruby-lang.org/) in your system is to follow the official [documentation](https://www.ruby-lang.org/en/documentation/installation/).

To ensure it is installed, let's run the following:

```bash
ruby --version # This will show the Ruby version
```

{% hint style="success" %}
If this command works, Ruby is successfully installed!
{% endhint %}

## Start a Ruby project

Now it's time to create a new project and add Wasmer as a dependency:

```bash
mkdir wasmer-project
cd wasmer-project
```

You can now prepare your `Gemfile`:

```ruby
source 'https://rubygems.org'
gem 'wasmer'
```

And install everything using Bundler:

```bash
bundle install
```

{% hint style="info" %}
Note that you can also install the Wasmer gem directly without using Bundler:

```bash
gem install wasmer
```
{% endhint %}

Next, let's take a look at some examples!

{% page-ref page="../examples/" %}


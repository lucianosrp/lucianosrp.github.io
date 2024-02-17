FROM ruby:alpine3.19

RUN apk add --no-cache build-base nodejs-current

RUN gem install bundler

WORKDIR /usr/src/app

COPY . /usr/src/app

RUN bundle install

CMD ["bundle"]
CMD ["bundle", "exec", "jekyll", "serve", "--host", "0.0.0.0", "--livereload"]

EXPOSE 4000


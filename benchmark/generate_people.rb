#!/usr/bin/env ruby
require 'faker'
require 'json'

trap 'PIPE' do
  exit 0
end
I18n.enforce_available_locales = false
include Faker
loop do
  puts JSON.dump({
    name: Name::name,
    email: Internet.email,
    phone: PhoneNumber.phone_number,
    ip: Internet.ip_v4_address,
    created_at: Time.now.to_i - rand(24 * 60 * 60 * 30),
  })
end

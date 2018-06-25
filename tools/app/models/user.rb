class User < ApplicationRecord
  has_many :logs
  has_many :policies
end

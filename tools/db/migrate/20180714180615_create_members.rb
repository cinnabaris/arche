class CreateMembers < ActiveRecord::Migration[5.2]
  def change
    create_table :members do |t|
      t.string :nick_name, null: false, limit: 255
      t.string :real_name, null: false, limit: 255
      t.string :phone, limit: 255
      t.string :email, limit: 255
      t.string :address, limit: 255
      t.string :line, limit: 255
      t.string :wechat, limit: 255
      t.string :skype, limit: 255
      t.string :weibo, limit: 255
      t.string :facebook, limit: 255
      t.timestamps
    end
    add_index :members, :nick_name, unique: true
    add_index :members, :real_name
  end
end

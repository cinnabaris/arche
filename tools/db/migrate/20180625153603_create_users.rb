class CreateUsers < ActiveRecord::Migration[5.2]
  def change
    create_table :users do |t|
      t.string :name, null: false, limit: 32
      t.string :email, null: false, limit: 255
      t.binary :password
      t.string :uid, null: false, limit: 36
      t.string :provider_type, null: false, limit: 16
      t.string :provider_id, null: false, limit: 255
      t.string :logo, null: false, limit: 255
      t.integer :sign_in_count, null: false, limit: 8
      t.datetime :current_sign_in_at
      t.string :current_sign_in_ip, limit: 39
      t.datetime :last_sign_in_at
      t.string :last_sign_in_ip, limit: 39
      t.datetime :confirmed_at
      t.datetime :locked_at
      t.datetime :deleted_at
      t.timestamps
    end
    add_index :users, :name
    add_index :users, :email
    add_index :users, %i[provider_type provider_id], unique: true
    add_index :users, :uid, unique: true
  end
end

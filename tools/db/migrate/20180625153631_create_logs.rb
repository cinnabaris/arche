class CreateLogs < ActiveRecord::Migration[5.2]
  def change
    create_table :logs do |t|
      t.references :user, null: false
      t.string :ip, null: false, limit: 39
      t.string :message, null: false, limit: 255
      t.datetime :created_at, null: false
    end
    add_index :logs, :ip
  end
end

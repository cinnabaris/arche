class CreateNotifications < ActiveRecord::Migration[5.2]
  def change
    create_table :notifications do |t|
      t.references :user, null: false
      t.string :url, null: false, limit: 255
      t.string :body, null: false, limit: 1024
      t.string :media_type, null: false, limit: 8
      t.string :level, null: false, limit: 1
      t.boolean :read, null: false
      t.timestamps
    end
    add_index :notifications, :level
  end
end

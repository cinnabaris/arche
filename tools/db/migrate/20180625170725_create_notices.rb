class CreateNotices < ActiveRecord::Migration[5.2]
  def change
    create_table :notices do |t|
      t.references :user, null: false
      t.string :title, null: false, limit: 255
      t.string :body, null: false, limit: 800
      t.string :media_type, null: false, limit: 8
      t.boolean :read, null: false
      t.timestamps
    end
    add_index :notices, :title
  end
end

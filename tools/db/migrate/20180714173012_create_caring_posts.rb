class CreateCaringPosts < ActiveRecord::Migration[5.2]
  def change
    create_table :caring_posts do |t|
      t.references :topic, null: false
      t.references :user, null: false
      t.string :method, null: false, limit: 255
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.datetime :begin, null: false
      t.datetime :end, null: false
      t.timestamps
    end
    add_index :caring_posts, :method
  end
end

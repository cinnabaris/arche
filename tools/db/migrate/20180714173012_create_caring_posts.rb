class CreateCaringPosts < ActiveRecord::Migration[5.2]
  def change
    create_table :caring_posts do |t|
      t.references :topic, null: false
      t.references :member, null: false
      t.string :who, null: false, limit: 255
      t.string :method, null: false, limit: 255
      t.string :address, null: false, limit: 255
      t.integer :progress, null: false, limit: 1
      t.string :timezone, null: false, limit: 64
      t.string :remind, limit: 500
      t.date :date, null: false
      t.time :begin, null: false
      t.time :end, null: false
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.timestamps
    end
    add_index :caring_posts, :timezone

    create_table :caring_posts_members do |t|
      t.references :post, null: false
      t.references :member, null: false
      t.datetime :created_at, null: false
    end
    add_index :caring_posts_members, %i[member_id post_id], unique: true
  end
end

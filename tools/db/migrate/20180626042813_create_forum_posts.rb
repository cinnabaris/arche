class CreateForumPosts < ActiveRecord::Migration[5.2]
  def change
    create_table :forum_posts do |t|
      t.references :user, null: false
      t.references :topic, null: false
      t.references :post
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.timestamps
    end
  end
end

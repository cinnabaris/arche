class CreateForumBadges < ActiveRecord::Migration[5.2]
  def change
    create_table :forum_badges do |t|
      t.text :body, null: false
      t.string :title, null: false, limit: 64
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.string :icon, null: false, limit: 32
      t.timestamps
    end
    add_index :forum_badges, :title
  end
end

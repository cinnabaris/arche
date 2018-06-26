class CreateForumCategories < ActiveRecord::Migration[5.2]
  def change
    create_table :forum_categories do |t|
      t.string :title, null: false, limit: 255
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.string :background, null: false, limit: 6
      t.string :foreground, null: false, limit: 6
      t.integer :position, null: false, limit: 1
      t.timestamps
    end
    add_index :forum_categories, :title
  end
end

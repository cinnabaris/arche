class CreateFriendLinks < ActiveRecord::Migration[5.2]
  def change
    create_table :friend_links do |t|
      t.string :title, null: false, limit: 32
      t.string :home, null: false, limit: 255
      t.string :logo, null: false, limit: 255
      t.integer :position, null: false, limit: 1
      t.timestamps
    end
    add_index :friend_links, :title
  end
end

class CreateCards < ActiveRecord::Migration[5.2]
  def change
    create_table :cards do |t|
      t.string :title, null: false, limit: 255
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.string :action, null: false, limit: 32
      t.string :href, null: false, limit: 255
      t.string :logo, null: false, limit: 255
      t.string :loc, null: false, limit: 16
      t.string :lang, null: false, limit: 8
      t.integer :position, null: false, limit: 1
      t.timestamps
    end
    add_index :cards, :lang
    add_index :cards, %i[lang loc]
  end
end

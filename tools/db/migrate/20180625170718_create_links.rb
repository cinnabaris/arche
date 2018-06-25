class CreateLinks < ActiveRecord::Migration[5.2]
  def change
    create_table :links do |t|
      t.string :href, null: false, limit: 255
      t.string :label, null: false, limit: 32
      t.string :loc, null: false, limit: 16
      t.string :lang, null: false, limit: 8
      t.integer :x, null: false, limit: 1
      t.integer :y, null: false, limit: 1
      t.timestamps
    end
    add_index :links, :lang
    add_index :links, %i[lang loc]
  end
end

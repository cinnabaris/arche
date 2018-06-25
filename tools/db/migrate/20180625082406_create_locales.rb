class CreateLocales < ActiveRecord::Migration[5.2]
  def change
    create_table :locales do |t|
      t.string :lang, null: false, limit: 8
      t.string :code, null: false, limit: 255
      t.text :message, null: false
      t.timestamps
    end
    add_index :locales, :lang
    add_index :locales, :code
    add_index :locales, %i[lang code], unique: true
  end
end

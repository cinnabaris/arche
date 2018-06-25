class CreateLocales < ActiveRecord::Migration[5.2]
  def change
    create_table :locales do |t|
      t.string :lang, null: false
      t.string :code, null: false
      t.text :message, null: false
      t.timestamps
    end
    add_index :locales, :lang
    add_index :locales, :code
    add_index :locales, %i[lang code], unique: true
  end
end

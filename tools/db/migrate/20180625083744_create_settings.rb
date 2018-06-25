class CreateSettings < ActiveRecord::Migration[5.2]
  def change
    create_table :settings do |t|
      t.string :key, null: false
      t.binary :value, null: false
      t.binary :salt
      t.timestamps
    end
    add_index :settings, :key, unique: true
  end
end

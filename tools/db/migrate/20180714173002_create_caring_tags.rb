class CreateCaringTags < ActiveRecord::Migration[5.2]
  def change
    create_table :caring_tags do |t|
      t.string :name, null: false, limit: 36
      t.timestamps
    end
    add_index :caring_tags, :name, unique: true
  end
end

class CreateRoles < ActiveRecord::Migration[5.2]
  def change
    create_table :roles do |t|
      t.string :name, null: false, limit: 32
      t.string :resource_type, limit: 255
      t.integer :resource_id, limit: 8
      t.datetime :created_at, null: false
    end
  end
end

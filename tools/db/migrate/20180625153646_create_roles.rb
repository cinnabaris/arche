class CreateRoles < ActiveRecord::Migration[5.2]
  def change
    create_table :roles do |t|
      t.string :name, null: false, limit: 32
      t.string :resource_type, null: false, limit: 255
      t.integer :resource_id, null: false, limit: 8
      t.datetime :created_at, null: false
    end
    add_index :roles, :resource_type
    add_index :roles, %i[resource_type resource_id]
    add_index :roles, %i[name resource_type resource_id], unique: true
  end
end

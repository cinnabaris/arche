class AddCreatedAtToSchemaMigrations < ActiveRecord::Migration[5.2]
  def change
    add_column :schema_migrations, :created_at, :datetime, null: false, default: -> { 'CURRENT_TIMESTAMP' }
  end
end
